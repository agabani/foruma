import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import PureHomeScreen from "../stories/screens/pure/PureHomeScreen.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Home",
    component: PureHomeScreen,
  },
  {
    path: "/about",
    name: "About",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
  },
  {
    path: "/account-settings",
    name: "AccountSettings",
    component: () =>
      import(
        /*  webpackChunkName: "account-settings" */ "../stories/screens/pure/PureAccountSettingsScreen.vue"
      ),
  },
  {
    path: "/login",
    name: "Login",
    component: () =>
      import(
        /* webpackChunkName: "login" */ "../stories/screens/pure/PureLoginScreen.vue"
      ),
  },
  {
    path: "/signup",
    name: "Signup",
    component: () =>
      import(
        /*  webpackChunkName: "signup" */ "../stories/screens/pure/PureSignupScreen.vue"
      ),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
