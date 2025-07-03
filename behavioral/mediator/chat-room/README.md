# Mediator Pattern - Chatroom Example 

## Project Overview

The **Mediator Pattern** is a behavioral design pattern that encapsulates how a set of objects interact. Instead of allowing objects to refer to each other directly, they communicate through a mediator object, promoting loose coupling and centralized control over interaction logic.

This project demonstrates the Mediator Pattern using a simple chatroom scenario. Users can send and receive messages through a central `ChatRoomMediator`, without referencing each other directly.

## Pattern Structure

- **Mediator**: Defines an interface for communication between users.
- **ConcreteMediator (ChatRoomMediator)**: Manages and coordinates registered users, dispatches messages.
- **Colleague (User)**: Participates in communication by sending and receiving messages through the mediator.
- **Client**: Registers users and triggers message exchange.

## Key Features

- Decouples users from one another via a mediator.
- Allows centralized message routing and optional logging.
- Easily scalable to support advanced chatroom features.

## Example Behavior

1. Users are registered with the `ChatRoomMediator`.
2. When a user sends a message, the mediator distributes it to other users.
3. Each user receives and handles the message individually.