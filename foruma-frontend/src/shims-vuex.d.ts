// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ComponentCustomProperties } from "vue";
import { Store } from "vuex";

declare module "@vue/runtime-core" {
  // declare your own store states

  // data
  interface Authentication {
    username: string;
  }

  // events
  interface PasswordChangedEvent {
    eventDate: Date;
    success: boolean;
    message: string | undefined;
  }

  interface ChangedEvent {
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
      loginChanged: ChangedEvent | undefined;
      signupChanged: ChangedEvent | undefined;
    };
  }

  // provide typings for `this.$store`
  interface ComponentCustomProperties {
    $store: Store<State>;
  }
}
