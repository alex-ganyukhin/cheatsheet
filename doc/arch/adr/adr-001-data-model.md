# Data Model

- **Status**: Accepted
- **Date**: 2025-11-02
- **Decision-maker**: Aleksandr Ganiukhin


## Context and Problem Statement

In the scope of [#1](https://github.com/alex-ganyukhin/cheatsheet/issues/1), we need to define a data model for storing and retrieving cheat sheets efficiently.
The primary requirement:
- For the MVP, we need to keep it simple and data model shall contain only essential fields to represent a cheat sheet.
- In future iterations (if needed), we can extend the data model to include additional metadata, such as tags, categories, and more.


## Considered Options

**id**
- name: id
- type: string
- description: Unique identifier for the cheat sheet
- status: Rejected
- justification: Overhead, we will use `title`

**title**
- name: title
- type: string
- description: Small and laconic title of the cheat sheet
- constraints:
  - Unique
  - Not null
  - Max length 100 characters
- status: Accepted

**command**
- name: command
- type: string
- description: The command or code snippet represented by the cheat sheet
- constraints:
  - Not null
  - Max length 5000 characters
- status: Accepted
- justification: 5000 characters is WAY above what we need for MVP, but gives us room to grow

**description**
- name: description
- type: string
- description: A brief description of what the command is about
- constraints:
  - Optional
  - Max length 10000 characters
- status: Accepted


## Decision Outcome

- **See** "Considered Options" section above for the final data model structure.
- In future releases additional fields may be required, including "dynamic fields", like "last_used", "times_used", etc, which cannot be effectively stored/updated in .toml files. Therefore, such dynamic fields will be stored separately.

### Consequences

#### Positive consequences

* The KISS data model is easy to implement and maintain, while still meeting the MVP requirements.


#### Negative consequences

* At the moment of creation no negative consequences
