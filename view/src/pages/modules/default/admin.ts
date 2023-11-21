import type { AppRouteModule } from '/@/router/types';

import { LAYOUT } from '/@/router/constant';

const empty: AppRouteModule = {
  path: '/admin',
  name: 'AdminPage',
  component: LAYOUT,
  redirect: '/admin/index',
  meta: {
    hideChildrenInMenu: true,
    icon: 'ion:menu-outline',
    title: "管理列表",
    orderNo: 100000,
  },
  children: [
    {
      path: 'index',
      name: 'AdminPage',
      component: () => import('/@/pages/view/AdminPage.vue'),
      meta: {
        title: "管理列表",
      }
    },
  ],
};

export default empty;
