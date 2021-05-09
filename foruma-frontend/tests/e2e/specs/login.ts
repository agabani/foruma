describe("Login", () => {
  it("logs in", () => {
    cy.visit("/");

    cy.contains("Login").click();
    cy.url().should("include", "/login");

    cy.get(".base-text-field")
      .type("test-username")
      .should("have.value", "test-username");

    cy.get(".base-password-field")
      .type("test-password")
      .should("have.value", "test-password");

    cy.get(".base-button").click();
  });
});
