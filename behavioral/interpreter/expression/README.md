# Mediator Pattern Example

The **Mediator Pattern** is a behavioral design pattern that reduces coupling between multiple objects. It introduces a mediator object that encapsulates how these objects interact, so that individual objects do not need to reference each other directly. This improves the modularity, maintainability, and flexibility of the system.

This project demonstrates the pattern with a "chatroom" scenario. Multiple users send and receive messages through a central mediator. Instead of direct communication between users, messages are passed to the mediator, which then dispatches them to the appropriate recipients.

### Pattern Structure

- **Mediator**: Defines a unified interface for communication and coordinates interactions between participants.
- **Concrete Mediator**: Implements the mediator interface, maintains a list of participants, and handles the message dispatch logic.
- **Colleague/User**: Interacts with the mediator to send and receive messages.
- **Client**: Responsible for creating the mediator and users, and initiating the communication process.

### Applicable Scenarios

- Decoupling GUI components and coordinating events (e.g., interaction between buttons, forms, and text fields)
- Message dispatching between users in chatrooms or collaboration tools
- Systems with complex interaction rules between components, where central coordination is desired

### Advantages

- Reduces coupling between components and clarifies system structure
- Centralized control makes it easier to add behavior such as logging or message filtering
- Simplifies maintenance and extension of interaction logic

### Disadvantages

- The mediator can become overly complex, evolving into a "god object"
- Centralizing all interaction logic can lead to potential performance bottlenecks
