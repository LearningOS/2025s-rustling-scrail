/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	active_1:bool,
    size:usize,
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			active_1:true,
            size:0,
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }

    fn get_rqs(&mut self) -> (&mut Queue<T>, &mut Queue<T>) {
        if self.active_1 {
            return (&mut self.q1, &mut self.q2)
        } else {
            return (&mut self.q2, &mut self.q1)
        }
    }

    pub fn push(&mut self, elem: T) {
        let (rq1,_) = self.get_rqs();
        rq1.enqueue(elem);
        self.size += 1;
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        let size = self.size;
        self.size -= 1;
        let (rq1, rq2) = self.get_rqs();
        for _ in 0..size-1 {
            rq2.enqueue(rq1.dequeue().unwrap());
        }
        let ret = rq1.dequeue().unwrap();
        self.active_1 = !self.active_1;
        Ok(ret)
		
    }
    pub fn is_empty(&self) -> bool {
		return self.size == 0;
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}