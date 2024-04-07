# Task 3

## Breakdown

- [ ] Break down what I need to do even further since I am struggling to
      comprehend what I am doing.
- [ ] Need to figure out how to store identifiers for new connections
      so that we don't duplicate connections?

When the `Manager` accepts a new connection, we can their id by using `recv.id()`

From the documentation of `send`, it may be that my issue is that I am not
establishing a channel per connection. This will lead to the SendError
that I am getting on subsequent messages from the same sender.

```
Attempts to send a value on this channel, returning it back if it could
not be sent.

A successful send occurs when it is determined that the other end of
the channel has not hung up already. An unsuccessful send would be one
where the corresponding receiver has already been deallocated. Note
that a return value of [`Err`](https://doc.rust-lang.org/stable/core/result/enum.Result.html) means that the data will never be
received, but a return value of [`Ok`](https://doc.rust-lang.org/stable/core/result/enum.Result.html) does *not* mean that the data
will be received. It is possible for the corresponding receiver to
hang up immediately after this function returns [`Ok`](https://doc.rust-lang.org/stable/core/result/enum.Result.html).

This method will never block the current thread.
```
