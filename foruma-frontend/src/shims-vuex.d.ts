// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ComponentCustomProperties } from "vue";
import { Store } from "vuex";

declare module "@vue/runtime-core" {
  // declare your own store states

  // state: authentication
  interface Authentication {
    username: string;
  }

  // event: password changed
  interface PasswordChangedEvent {
    when: Date;
    success: boolean;
    message: string | undefined;
  }

  // event: login
  interface LoginChangedEvent {
    eventDate: Date;
    error:
      | {
          title: string;
          message: string;
        }
      | undefined;
  }

  // event: signup
  interface SignupChangedEvent {
    eventDate: Date;
    error:
      | {
          title: string;
          message: string;
        }
      | undefined;
  }

  interface State {
    data: {
      authentication: Authentication | undefined;
    };
    events: {
      passwordChanged: PasswordChangedEvent | undefined;
      loginChanged: LoginChangedEvent | undefined;
      signupChanged: SignupChangedEvent | undefined;
    };
  }

  // provide typings for `this.$store`
  interface ComponentCustomProperties {
    $store: Store<State>;
  }
}
