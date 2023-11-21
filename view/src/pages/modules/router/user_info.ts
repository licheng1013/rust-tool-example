import type { AppRouteModule } from '/@/router/types';

import { LAYOUT } from '/@/router/constant';

const UserInfo: AppRouteModule = {
  path: '/UserInfo',
  name: 'UserInfoView',
  component: LAYOUT,
  redirect: '/UserInfo',
  meta: {
    hideChildrenInMenu: true,
    orderNo: 11,
    icon: 'mdi:user-outline',
    title: "模板",
  },
  children: [
    {
      path: '',
      name: 'UserInfoView',
      component: () => import('/@/pages/view/UserInfoPage.vue'),
      meta: {
        title: "模板",
        hideMenu: true,
      },
    },
  ],
};

export default UserInfo;
