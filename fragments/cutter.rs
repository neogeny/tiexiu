pub trait Cutter {
    fn take_cut(&mut self) -> bool {
        false
    }

    fn setcut(&mut self) {
    }
}


impl Cutter for Nope {
    fn take_cut(&mut self) -> bool {
        let was_cut = self.cutseen;
        self.cutseen = false;
        was_cut
    }

    fn setcut(&mut self) {
        self.cutseen = true;
    }
}

