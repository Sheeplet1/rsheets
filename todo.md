# TODO

- [ ] Need to create a worker thread which handles updating dependencies
      as per the spec.

1. Create an extra thread outside of the ThreadPool which will handle all
   dependency updates.
2. Create a queue which will hold all worker commands sent from the other threads.
3. While there is commands in the queue, the worker thread should be working.
   The worker thread should only close once the program has exited.
