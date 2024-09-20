Feature: Login feature

  Scenario Outline: Test Login with valid credentials
    Given User navigates to login page
    When User enters <username> and <password>
    And Click on login button
    Then User is navigated to home page
    
    Examples:
    | username | password |
    | Ravikant | 12345 		|
    | Chandu	 | 12345    |
    