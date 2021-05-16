describe("authentication", () => {
  it("logs in and out", () => {
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
});
