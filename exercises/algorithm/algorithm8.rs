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
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    // Ensure stack order after each push operation
    // Use cache queue for O(sqrt(n)) time consumption in push
    pub fn push(&mut self, elem: T) {
        // Get current size of each queue
        let size1 = self.q1.size();
        let size2 = self.q2.size();
        // Push into q1 directly when q1 is short
        if size1.pow(2) <= size2 {
            // Enqueue in q1
            self.q1.enqueue(elem);
            // Rearrange original elems q1 in stack order
            for _ in 0..size1 {
                let front_elem = self.q1.dequeue().unwrap();
                self.q1.enqueue(front_elem);
            }
        // Cache into q2 when q1 is too long
        } else {
            // Enqueue in q2
            self.q2.enqueue(elem);
            // Move all elems in q1 into q2
            for _ in 0..size1 {
                let front_elem = self.q1.dequeue().unwrap();
                self.q2.enqueue(front_elem);
            }
            // Rearrange original elems in q2 in stack order
            for _ in 0..size2 {
                let front_elem = self.q2.dequeue().unwrap();
                self.q2.enqueue(front_elem);
            }
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        match self.q1.is_empty() {
            true => match self.q2.is_empty() {
                true => Err("Stack is empty"),
                false => self.q2.dequeue(),
            }
            false => self.q1.dequeue(),
        }
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty() && self.q2.is_empty()
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