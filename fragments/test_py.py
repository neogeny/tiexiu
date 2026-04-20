import sys
sys.path.insert(0, 'tatsu_cloned')
from tatsu.tool import compile
from tatsu.railroads import draw

g = '''start = 'a' | 'b' 'c' 'd' ;'''
model = compile(g)
draw(model)