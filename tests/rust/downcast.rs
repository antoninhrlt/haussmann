use std::any::Any;

trait B {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Eq, PartialEq)]
struct A {

}

impl B for A {
    fn as_any(&self) -> &dyn Any {
       self 
    }
}

#[test]
fn downcast() {
    let a = A {};
    
    // A -> dyn B
    let bees: Vec<Box<dyn B>> = vec![Box::new(a)];

    let a2 = bees[0].as_any().downcast_ref::<A>();
    assert_ne!(a2, None);
    assert_eq!(a2, Some(&A {}));
}

#[test]
#[should_panic]
fn downcast_panicking() {
    let a = A {};
    
    // A -> dyn B
    let bees: Vec<Box<dyn B>> = vec![Box::new(a)];

    // ref
    let a2 = (&bees[0] as &dyn Any).downcast_ref::<A>();
    assert_ne!(a2, None);
    assert_eq!(a2, Some(&A {}));
}

#[test]
fn any() {
    fn as_any<T: 'static>(t: &T) -> &dyn Any {
        t 
    }

    let b: Box<dyn B> = Box::new(A{});
    
    let any_b1 = as_any(&b);
    let any_b2 = &b as &dyn Any;
    let any_b3: &dyn Any = &b;
}