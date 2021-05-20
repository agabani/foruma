describe("authentication", () => {
  it("should sign up, log out, log in, log out", () => {
    const username = Cypress._.uniqueId(Date.now().toString());
    const password = Cypress._.uniqueId(Date.now().toString());

    cy.visit("/");

    // sign up
    cy.get(".base-header").contains("Sign up").click();

    cy.url().should("include", "/signup");

    cy.get('.base-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.base-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).type(password).should("have.value", password);
    });

    cy.get('.base-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    // log out
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Sign up");

    // log in
    cy.get(".base-header").contains("Log in").click();

    cy.url().should("include", "/login");

    cy.get('.base-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.base-login-form input[type="password"]')
      .type(password)
      .should("have.value", password);

    cy.get('.base-login-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    // log lout
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Log in");
  });
});
