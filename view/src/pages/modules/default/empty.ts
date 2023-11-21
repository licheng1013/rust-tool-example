import type { AppRouteModule } from '/@/router/types';

import { LAYOUT } from '/@/router/constant';

const empty: AppRouteModule = {
  path: '/empty',
  name: 'EmptyPage',
  component: LAYOUT,
  redirect: '/empty/index',
  meta: {
    hideChildrenInMenu: true,
    icon: 'ion:aperture-outline',
    title: "空页面",
    orderNo: 100000,
  },
  children: [
    {
      path: 'index',
      name: 'EmptyPage',
      component: () => import('/@/pages/view/EmptyPage.vue'),
      meta: {
        title: "空页面",
      }
    },
  ],
};

export default empty;
