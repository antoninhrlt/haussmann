use std::any::Any;

trait B {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct A<T: 'static> {
    value: Box<T>,
    closure: fn(value: &mut T),
}

impl<T> B for A<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T> A<T> {
    fn call_closure(&mut self) {
        let fn_closure = self.closure;
        fn_closure(&mut self.value);
    }
}

struct V {
    bees: Vec<Box<dyn B>>,
}

fn foo(v: &V) {}

#[test]
fn closures() {
    let mut v: V = V {
        bees: vec![
            Box::new(
                A {
                    value: Box::new(1i32),
                    closure: |t: &mut i32| {
                        *t += 1;
                        println!("{}", t);
                    },
                }
            )
        ]
    };

    loop {
        foo(&v);
        
        let a: &mut A<i32> = v.bees[0]
            .as_any_mut()
            .downcast_mut::<A<i32>>()
            .unwrap();
        
        a.call_closure();
    }
}
