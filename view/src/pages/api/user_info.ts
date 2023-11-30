import { ErrorMessageMode } from "#/axios";
import { defHttp } from "@/utils/http/axios";

// 列表
export function userInfoList(params: any) {
  return defHttp.get<any>({ url: "/user/info/list", params });
}

// 删除
export function userInfoDelete(params: any) {
  return defHttp.post<any>({ url: "/user/info/delete", params });
}

// 新增
export function userInfoInsert(params: any) {
  return defHttp.post<any>({ url: "/user/info/insert", params });
}

// 修改
export function userInfoUpdate(params: any) {
  return defHttp.post<any>({ url: "/user/info/update", params });
}
