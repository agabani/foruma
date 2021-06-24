describe("session", () => {
  it("test", () => {
    const username = Cypress._.uniqueId(Date.now().toString());
    const password = Cypress._.uniqueId(Date.now().toString());

    cy.visit("/");

    // sign up
    cy.get(".base-header").contains("Sign up").click();

    cy.url().should("include", "/signup");

    cy.get('.pure-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).type(password).should("have.value", password);
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
    cy.get('.pure-login-form button[type="submit"]').click();
    cy.get('.pure-login-form button[type="submit"]').click();
    cy.get('.pure-login-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    cy.wait(1000);

    // view sessions
    cy.get(".base-header").contains(username).click();

    cy.get(".pure-session-card").should("have.length", 4);

    // delete non current sessions
    cy.get(".pure-session-card")
      .not(':contains("current session")')
      .find(".pure-session-card--trash")
      .click({ multiple: true });

    cy.reload();

    cy.get(".base-header").contains(username).click();
    cy.get(".pure-session-card").should("have.length", 1);

    // delete remaining sessions
    cy.get(".pure-session-card--trash").click();

    cy.reload();

    cy.get(".base-header").contains("Sign up");
  });
});
