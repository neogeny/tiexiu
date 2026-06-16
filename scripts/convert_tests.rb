# Converts json crate macros to serde_json in test files.
# Handles: value!(), array![], array!(), array! [] (with space),
#          object!{}, object!(), object! {} (with space)
# Adds `use serde_json::json;` to converted files.

files = Dir.glob("tests/*_test.rs")

files.each do |f|
  content = File.read(f)
  orig = content.dup

  had_json_import = content.match?(/^#\[macro_use\]\n|^extern crate json;|^use json::/)

  # Step 1: Remove json crate imports
  content.sub!(/^#\[macro_use\]\n/, "")
  content.gsub!(/^extern crate json;\n/, "")
  content.gsub!(/^use json::.*;\n/, "")

  # Step 2: Convert value!() → json!()
  content.gsub!("value!(", "json!(")

  # Step 3: Convert array! and object! macros
  result = ""
  i = 0
  len = content.length

  json_close_stack = []
  macro_close_stack = []
  in_string = false

  skip_ws = ->(start) {
    p = start
    while p < len && (content[p] == ' ' || content[p] == "\t")
      p += 1
    end
    p
  }

  while i < len
    if content[i] == '"' && (i == 0 || content[i-1] != '\\')
      in_string = !in_string
      result << content[i]
      i += 1
      next
    end

    unless in_string
      # Check for array! at current position
      if i + 5 < len && content[i..i+5] == "array!" && i + 6 < len
        next_pos = skip_ws.call(i + 6)
        if next_pos < len && (content[next_pos] == '[' || content[next_pos] == '(')
          open = content[next_pos]
          result << "json!([" + content[(i+6)...next_pos]
          macro_close_stack.push(open == '[' ? ']' : ')')
          json_close_stack.push(']')
          i = next_pos + 1
          next
        end
      end

      # Check for object! at current position
      if i + 6 < len && content[i..i+6] == "object!" && i + 7 < len
        next_pos = skip_ws.call(i + 7)
        if next_pos < len && (content[next_pos] == '{' || content[next_pos] == '(')
          open = content[next_pos]
          result << "json!({" + content[(i+7)...next_pos]
          macro_close_stack.push(open == '{' ? '}' : ')')
          json_close_stack.push('}')
          i = next_pos + 1
          next
        end
      end

      if !macro_close_stack.empty? && content[i] == macro_close_stack.last
        macro_close_stack.pop
        jc = json_close_stack.pop
        result << jc.to_s + ")"
        i += 1
        next
      end
    end

    result << content[i]
    i += 1
  end

  content = result

  if had_json_import && !content.include?("use serde_json::json;")
    # Add use serde_json::json; after the first use tiexiu line, or at top
    if content =~ /^(use tiexiu)/
      content.sub!(/^(use tiexiu.*;)/, "\\1\nuse serde_json::json;")
    elsif content =~ /^(use \w+)/
      content.sub!(/^(use \w+.*;)/, "\\1\nuse serde_json::json;")
    else
      content = "use serde_json::json;\n" + content
    end
  end

  if content != orig
    File.write(f, content)
    puts "Updated: #{f}"
  end
end
