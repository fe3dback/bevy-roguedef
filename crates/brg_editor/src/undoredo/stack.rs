use anyhow::Result;
use strum::Display;

#[derive(Eq, PartialEq, Debug, Clone, Display)]
pub enum StackError {
    /// cannot write new element
    AllStackMemoryContainsNotExecutedCommands,
    /// cannot undo, because elements not executed and will be lost
    StackIsDirty,
    /// all stack capacity is undone
    AllElementsInStackAlreadyUndo,
    /// stack not contain any undo elements
    NothingToRedo,
}

pub struct Stack<T: Default> {
    elements: Vec<T>,
    count:    usize, // how many elements is pushed into stack since creation ("undo" will decrease this)
    executed: usize, // how many elements read from stack to initial execute command ("undo" will decrease this)
    undos:    usize, // how many elements in undo from top of stack. (we can "redo" exactly [undos] count of time back)
    ptr_e:    usize, // execute pointer (point to last executed elem index on circle stack)
    ptr_w:    usize, // write pointer (point to last written elem index on circle stack)
}

impl<T: Default> Stack<T> {
    pub fn new(capacity: usize) -> Self {
        // init memory
        let mut elements: Vec<T> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            elements.push(T::default());
        }

        // create
        Self {
            elements,
            count: 0,
            executed: 0,
            undos: 0,
            ptr_e: 0,
            ptr_w: 0,
        }
    }

    /// append not executed element to stack
    /// you need to call [execute] after this call, to get this
    /// element back and execute it
    pub fn write(&mut self, element: T) -> Result<(), StackError> {
        let capacity = self.elements.capacity();
        let limit = self.executed + capacity;

        if self.count >= limit {
            return Err(StackError::AllStackMemoryContainsNotExecutedCommands);
        }

        // reset undo history (cant redo since this)
        self.undos = 0;

        // write
        self.count += 1;
        self.ptr_w = self.next(self.ptr_w);
        self.elements[self.ptr_w] = element;

        // result
        Ok(())
    }

    /// returns list of all not executed elements
    /// and move ptr to tail. Second call since [write] will return empty vec.
    pub fn drain_not_executed(&mut self) -> Vec<&T> {
        // how many elements need to execute
        let need_to_exec = self.count - self.executed;

        // nothing to-do
        if need_to_exec == 0 {
            return vec![];
        }

        // create vec
        let mut result: Vec<&T> = Vec::with_capacity(need_to_exec);

        for _ in 0..need_to_exec {
            self.executed += 1;
            self.ptr_e = self.next(self.ptr_e);
            result.push(&self.elements[self.ptr_e]);
        }

        // return
        result
    }

    /// get last tail element for undo execution
    /// and lover all stack pointers by 1
    /// this action can be reverted with [redo]
    /// when [write] function is called, all undo ptr
    /// will be cleared
    pub fn undo(&mut self) -> Result<&T, StackError> {
        // stack is dirty, we need execute all elements first,
        // or they will be lost
        if self.executed < self.count {
            return Err(StackError::StackIsDirty);
        }

        if self.undos >= self.elements.capacity() {
            return Err(StackError::AllElementsInStackAlreadyUndo);
        }

        if self.count <= 0 {
            return Err(StackError::AllElementsInStackAlreadyUndo);
        }

        // undo
        let elem = &self.elements[self.ptr_w];

        self.undos += 1;
        self.ptr_e = self.prev(self.ptr_e);
        self.ptr_w = self.prev(self.ptr_w);
        self.count -= 1;
        self.executed -= 1;

        //

        Ok(elem)
    }

    /// redo last undo element
    /// this action can be called multiple times, each
    /// time ptr will be moved up in stack, until tail is reached
    pub fn redo(&mut self) -> Result<&T, StackError> {
        if self.undos <= 0 {
            return Err(StackError::NothingToRedo);
        }

        // redo
        self.undos -= 1;
        self.ptr_e = self.next(self.ptr_e);
        self.ptr_w = self.next(self.ptr_w);
        self.count += 1;
        self.executed += 1;
        let elem = &self.elements[self.ptr_w];

        Ok(elem)
    }

    #[inline]
    fn next(&self, ptr: usize) -> usize {
        match ptr >= self.elements.capacity() - 1 {
            true => 0,
            false => ptr + 1,
        }
    }

    #[inline]
    fn prev(&self, ptr: usize) -> usize {
        match ptr == 0 {
            true => self.elements.capacity() - 1,
            false => ptr - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_creation() {
        let mut stack = Stack::<i32>::new(3);
        assert_eq!(stack.elements.capacity(), 3);
        assert_eq!(stack.elements.len(), 3);
        assert_eq!(stack.count, 0);
        assert_eq!(stack.executed, 0);
        assert_eq!(stack.undos, 0);

        let res = stack.write(1);
        assert!(res.is_ok());
        assert_eq!(stack.elements.capacity(), 3);
        assert_eq!(stack.elements.len(), 3);
        assert_eq!(stack.count, 1);
        assert_eq!(stack.executed, 0);
        assert_eq!(stack.undos, 0);
    }

    #[test]
    pub fn test_write() {
        let mut stack = Stack::<i32>::new(2);

        let res = stack.write(1);
        assert!(res.is_ok());

        let res = stack.write(2);
        assert!(res.is_ok());

        let res = stack.write(3);
        assert_eq!(
            res.unwrap_err(),
            StackError::AllStackMemoryContainsNotExecutedCommands
        );
    }

    #[test]
    pub fn test_write_and_execute() {
        let mut stack = Stack::<i32>::new(2);

        _ = stack.write(1);
        _ = stack.write(2);
        assert_eq!(stack.drain_not_executed(), vec![&1, &2]);
        assert_eq!(stack.drain_not_executed(), Vec::<&i32>::new());

        _ = stack.write(3);
        _ = stack.write(4);
        assert_eq!(stack.drain_not_executed(), vec![&3, &4]);
        assert_eq!(stack.drain_not_executed(), Vec::<&i32>::new());
    }

    #[test]
    pub fn test_write_and_undo() {
        let mut stack = Stack::<i32>::new(2);

        // nothing to undo
        assert_eq!(stack.undo(), Err(StackError::AllElementsInStackAlreadyUndo));

        // add dirty elements
        _ = stack.write(1);
        _ = stack.write(2);

        // can`t undo dirty
        assert_eq!(stack.undo(), Err(StackError::StackIsDirty));

        // mark as clear
        _ = stack.drain_not_executed();

        // now we can undo in back order
        assert_eq!(stack.undo(), Ok(&2));
        assert_eq!(stack.undo(), Ok(&1));
        assert_eq!(stack.undo(), Err(StackError::AllElementsInStackAlreadyUndo));
    }

    #[test]
    pub fn test_write_will_clear_undo_ptr() {
        let mut stack = Stack::<i32>::new(2);
        _ = stack.write(1);
        _ = stack.write(2);
        _ = stack.drain_not_executed();

        // can undo last elem
        assert_eq!(stack.undo(), Ok(&2));

        // then write new elem (this will override ELEM=2, and clear undo pointer)
        _ = stack.write(4);
        assert_eq!(stack.undo(), Err(StackError::StackIsDirty));
        _ = stack.drain_not_executed();

        // test undo
        assert_eq!(stack.undo(), Ok(&4));
        assert_eq!(stack.undo(), Ok(&1));
        assert_eq!(stack.undo(), Err(StackError::AllElementsInStackAlreadyUndo));
    }

    #[test]
    pub fn test_redo() {
        let mut stack = Stack::<i32>::new(3);
        _ = stack.write(1);
        _ = stack.write(2);
        _ = stack.drain_not_executed();
        _ = stack.write(3);
        _ = stack.write(4);
        _ = stack.write(5);
        _ = stack.drain_not_executed();
        assert_eq!(stack.redo(), Err(StackError::NothingToRedo));

        // can undo last elem
        assert_eq!(stack.undo(), Ok(&5));

        // and can redo it again
        assert_eq!(stack.redo(), Ok(&5));

        // can more same tests
        assert_eq!(stack.undo(), Ok(&5));
        assert_eq!(stack.redo(), Ok(&5));

        assert_eq!(stack.undo(), Ok(&5));
        assert_eq!(stack.redo(), Ok(&5));

        assert_eq!(stack.undo(), Ok(&5));
        assert_eq!(stack.redo(), Ok(&5));

        // undo to head
        assert_eq!(stack.undo(), Ok(&5));
        assert_eq!(stack.undo(), Ok(&4));
        assert_eq!(stack.undo(), Ok(&3));

        // redo to tail
        assert_eq!(stack.redo(), Ok(&3));
        assert_eq!(stack.redo(), Ok(&4));
        assert_eq!(stack.redo(), Ok(&5));

        // can`t redo
        assert_eq!(stack.redo(), Err(StackError::NothingToRedo));
    }
}
