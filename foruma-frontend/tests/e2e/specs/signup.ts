describe("signup", () => {
  it("should sign up", () => {
    const username = Cypress._.uniqueId(Date.now().toString());
    const password = Cypress._.uniqueId(Date.now().toString());

    cy.visit("/");

    // sign up
    cy.get(".base-header").contains("Sign up").click();
    cy.url().should("include", "/signup");

    // try username only
    cy.get('.pure-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains("All fields needs to be filled in order to create an account!");

    cy.get(".base-alert--close").click();

    // try mismatched passwords
    cy.get('.pure-signup-form input[type="password"]').each(
      (element, index) => {
        cy.wrap(element)
          .type(`${password}${index}`)
          .should("have.value", `${password}${index}`);
      }
    );

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains("Make sure your passwords match then try again!");

    cy.get(".base-alert--close").click();

    // try matching passwords
    cy.get('.pure-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).clear().type(password).should("have.value", password);
    });

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);
  });

  it("should not be able to sign up with an existing account", () => {
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
      cy.wrap(element).clear().type(password).should("have.value", password);
    });

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains(username);

    // logout
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Sign up");

    // retsart
    cy.visit("/");

    // sign up
    cy.get(".base-header").contains("Sign up").click();
    cy.url().should("include", "/signup");

    cy.get('.pure-signup-form input[type="text"]')
      .type(username)
      .should("have.value", username);

    cy.get('.pure-signup-form input[type="password"]').each((element) => {
      cy.wrap(element).clear().type(password).should("have.value", password);
    });

    cy.get('.pure-signup-form button[type="submit"]').click();

    cy.contains("Uh oh, something went wrong");
    cy.contains("Sorry! There was a problem with your request!");
  });
});
