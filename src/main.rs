extern crate scoped_threadpool;
fn main() {
    let mut array = [0; 3];
    let mut pool = scoped_threadpool::Pool::new(2);
    pool.scoped(|scope| {
        //let array = &mut array;
        scope.execute(move||{
            array[0] = 1;
        });
        scope.execute(move||{
            array[0] = 2;
        });
    });
    println!("{:?}",array[0]);
}
