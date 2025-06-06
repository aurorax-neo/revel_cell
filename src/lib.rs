#![allow(unused)]
pub mod arc;
pub mod weak;

#[cfg(test)]
mod tests {
    use crate::arc::Arc;
    use crate::weak::Weak;

    #[test]
    fn arc_001() {
        let arc = Arc::<i32>::new(42);
        println!("{:?}", arc.inner_ptr());

        let weak = arc.downgrade();
        println!("{:?}", weak.inner_ptr());

        let arc_clone = arc.clone();
        println!("{:?}", arc == arc_clone);
        println!("{:?}", arc.eq_weak(&weak));
        println!("{:?}", weak.eq_arc(&arc));
        
        println!("strong_count = {} weak_count = {}", arc.strong_count(), arc.weak_count());
        println!("strong_count = {} weak_count = {}", weak.strong_count(), weak.weak_count());
    }

    #[test]
    fn weak_001() {
        let weak = Weak::<i32>::new();
        println!("{:?}", weak.inner_ptr());
    }
}
