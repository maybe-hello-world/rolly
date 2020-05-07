pub trait Policy<O, R>
{
    fn execute(&self, operation: O) -> R;
}