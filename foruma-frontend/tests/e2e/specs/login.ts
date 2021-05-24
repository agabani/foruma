describe("login", () => {
  it("should login", () => {
    const username = Cypress._.uniqueId(Date.now().toString());
    const password = Cypress._.uniqueId(Date.now().toString());

    cy.visit("/");

    // signup
    cy.get(".base-header").contains("Sign up").click();
    cy.url().should("include", "/signup");

    cy.get('.pure-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).clear().type(password).should("have.value", password);
    });

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    // logout
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Sign up");

    // login
    cy.get(".base-header").contains("Log in").click();
    cy.url().should("include", "/login");

    // try username only
    cy.get('.pure-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-login-form button[type="submit"]').click();

    cy.contains("Sorry! There was a problem with your request!");
    cy.contains(
      "All fields needs to be filled in order to login to an account!"
    );

    cy.get(".base-alert--close").click();

    // try matching passwords
    cy.get('.pure-login-form input[type="password"]')
      .type(password)
      .should("have.value", password);

    cy.get('.pure-login-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);
  });

  it("should not be able to login with incorrect password", () => {
    const username = Cypress._.uniqueId(Date.now().toString());
    const password = Cypress._.uniqueId(Date.now().toString());

    cy.visit("/");

    // signup
    cy.get(".base-header").contains("Sign up").click();
    cy.url().should("include", "/signup");

    cy.get('.pure-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).clear().type(password).should("have.value", password);
    });

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    // logout
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Sign up");

    // login
    cy.get(".base-header").contains("Log in").click();
    cy.url().should("include", "/login");

    cy.get('.pure-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    const wrongPassword = Cypress._.uniqueId(Date.now().toString());

    cy.get('.pure-login-form input[type="password"]')
      .type(wrongPassword)
      .should("have.value", wrongPassword);

    cy.get('.pure-login-form button[type="submit"]').click();

    cy.contains("Sorry! There was a problem with your request!");
    cy.contains("Sorry! There was a problem with your request!");

    cy.get(".base-alert--close").click();

    cy.get(".base-header").contains("Log in");
  });
});
