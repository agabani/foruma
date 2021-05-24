describe("authentication", () => {
  it("should sign up, log out, log in, change password, log out, log in", () => {
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

    cy.get(".base-header").contains(username);

    // change password
    const newPassword = Cypress._.uniqueId(Date.now().toString());

    cy.get(".base-header").contains(username).click();

    cy.get('.change-password-panel [type="password"]').each(
      (element, index) => {
        cy.wrap(element)
          .type(index ? newPassword : password)
          .should("have.value", index ? newPassword : password);
      }
    );

    cy.get('.change-password-panel button[type="submit"]').click();

    cy.contains("Your password has been changed");

    // log out
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Log in");

    // log in
    cy.get(".base-header").contains("Log in").click();

    cy.url().should("include", "/login");

    cy.get('.pure-login-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-login-form input[type="password"]')
      .type(newPassword)
      .should("have.value", newPassword);

    cy.get('.pure-login-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);
  });
});
