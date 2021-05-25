describe("change password", () => {
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

    // try current password only
    cy.get('.pure-change-password-panel [type="password"]')
      .first()
      .type(password)
      .should("have.value", password);

    cy.get('.pure-change-password-panel button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains(
      "All fields needs to be filled in order to change your password!"
    );

    cy.get(".base-alert--close").click();

    // try mismatched passwords
    cy.get('.pure-change-password-panel [type="password"]').each(
      (element, index) => {
        if (index > 0) {
          cy.wrap(element)
            .type(`${password}${index}`)
            .should("have.value", `${password}${index}`);
        }
      }
    );

    cy.get('.pure-change-password-panel button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains("Make sure your passwords match then try again!");

    cy.get(".base-alert--close").click();

    // try incorrect current password
    cy.get('.pure-change-password-panel [type="password"]').each((element) => {
      cy.wrap(element)
        .clear()
        .type(newPassword)
        .should("have.value", newPassword);
    });

    cy.get('.pure-change-password-panel button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains("Your current password is incorrect!");

    cy.get(".base-alert--close").click();

    // try correct current password
    cy.get('.pure-change-password-panel [type="password"]')
      .first()
      .clear()
      .type(password)
      .should("have.value", password);

    cy.get('.pure-change-password-panel button[type="submit"]').click();

    cy.contains("Your password has been changed");

    cy.get(".base-alert--close").click();

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
