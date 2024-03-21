use once_cell::unsync::OnceCell;

pub struct DataElementLocation<ReturnType>
{
    value: OnceCell<ReturnType>,
    reader: Box<dyn Fn() -> ReturnType>
}

impl<ReturnType> DataElementLocation<ReturnType>
where ReturnType: Clone
{
    pub fn read_value(&self) -> ReturnType
    {
        self.value.get_or_init(|| (self.reader)()).clone()
    }

    pub fn new(reader: Box<dyn Fn() -> ReturnType>) -> Self
    {
        Self { value: OnceCell::default(), reader }
    }
}