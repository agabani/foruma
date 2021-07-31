// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ComponentCustomProperties } from "vue";
import { Store } from "vuex";

declare module "@vue/runtime-core" {
  // declare your own store states

  // data
  interface Authentication {
    username: string;
    sessions: AuthenticationSession[];
  }

  interface Forum {
    subForums: SubForum[];
  }

  interface SubForum {
    id: string;
    name: string;
  }

  interface AuthenticationSession {
    id: string;
    isCurrentSession: boolean;
    browser: string | null;
    location: string | null;
    operatingSystem: string | null;
    lastActiveDate: Date;
  }

  // events
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
      forum: Forum | undefined;
    };
    events: {
      passwordChanged: ChangedEvent | undefined;
      loginChanged: ChangedEvent | undefined;
      signupChanged: ChangedEvent | undefined;
    };
  }

  // provide typings for `this.$store`
  interface ComponentCustomProperties {
    $store: Store<State>;
  }
}
