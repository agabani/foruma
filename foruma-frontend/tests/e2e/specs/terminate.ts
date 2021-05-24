describe("terminate", () => {
  it("should sign up, log out, log in, log out, log in, terminate account", () => {
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

    // log out
    cy.get(".base-header").contains("Log out").click();
    cy.get(".base-header").contains("Sign up");

    // log in
    cy.get(".base-header").contains("Log in").click();
    cy.url().should("include", "/login");

    cy.get('.pure-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-login-form input[type="password"]')
      .type(password)
      .should("have.value", password);

    cy.get('.pure-login-form button[type="submit"]').click();

    // terminate account
    cy.get(".base-header").contains(username).click();

    cy.contains("Delete account").click();

    cy.get(".base-header").contains("Log in");

    // login
    cy.get(".base-header").contains("Log in").click();
    cy.url().should("include", "/login");

    cy.get('.pure-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-login-form input[type="password"]')
      .type(password)
      .should("have.value", password);

    cy.get('.pure-login-form button[type="submit"]').click();

    cy.contains("Sorry! There was a problem with your request!");
    cy.contains("Sorry! There was a problem with your request!");

    cy.get(".base-alert--close").click();

    cy.get(".base-header").contains("Log in");
  });
});
