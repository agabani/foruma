describe("authentication", () => {
  it("logs in then logs out", () => {
    cy.visit("/");

    cy.get(".base-header").contains("Log in").click();

    cy.url().should("include", "/login");

    cy.get('.base-login-form input[type="text"]')
      .type("test-username")
      .should("have.value", "test-username");

    cy.get('.base-login-form input[type="password"]')
      .type("test-password")
      .should("have.value", "test-password");

    cy.get('.base-login-form button[type="submit"]').click();

    cy.get(".base-header").contains("test-username");
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Log in");
  });

  it("signs up then logs out", () => {
    cy.visit("/");

    cy.get(".base-header").contains("Sign up").click();

    cy.url().should("include", "/signup");

    cy.get('.base-signup-form input[type="text"]')
      .type("test-new-username")
      .should("have.value", "test-new-username");

    cy.get('.base-signup-form input[type="password"]').each((element) => {
      cy.wrap(element)
        .type("test-new-password")
        .should("have.value", "test-new-password");
    });

    cy.get('.base-signup-form button[type="submit"]').click();

    cy.get(".base-header").contains("test-new-username");
    cy.get(".base-header").contains("Log out").click();

    cy.get(".base-header").contains("Sign up");
  });
});
