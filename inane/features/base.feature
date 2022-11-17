Feature: Base Tests

    Scenario: Default message
        When I run my program
        Then I should see "Hello World" in stdout

    Scenario: Custom message
        Given I have the environment variables:
            | INANE_MESSAGE | Hullo |
        When I run my program
        Then I should see "Hullo" in stdout
