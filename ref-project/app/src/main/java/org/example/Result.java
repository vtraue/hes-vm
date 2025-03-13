package org.example;

class ErrorValueUnwrappedException extends RuntimeException {
  public ErrorValueUnwrappedException() {}

  public ErrorValueUnwrappedException(Object errorObj) {
    super(String.format("Called unwrap on error value: %s", errorObj.toString()));  
  }
}

class OkValueUnwrappedException extends RuntimeException {
  public OkValueUnwrappedException() {}

  public OkValueUnwrappedException(Object okObj) {
    super(String.format("Called getErr on ok value: %s", okObj.toString()));  
  }
}

public sealed interface Result<T, E> {
  T unwrap() throws ErrorValueUnwrappedException;
  boolean isOk();
    E getErr() throws OkValueUnwrappedException;
}

record Ok<T, E>(T val) implements Result<T, E> {
  @Override
  public T unwrap() throws ErrorValueUnwrappedException {
    return this.val;
  }
  public boolean isOk() {
    return true;
  }
  public E getErr() {
    throw new OkValueUnwrappedException();  
  }
}
record Err<T, E>(E err) implements Result<T, E> {
  @Override
  public T unwrap() throws ErrorValueUnwrappedException {
    throw new ErrorValueUnwrappedException(this.err);
  }
  public boolean isOk() {
    return false;
  }
  
  public E getErr() {
    return err;
  }
}
