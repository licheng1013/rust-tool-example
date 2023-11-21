<template>
  <div>
    <BasicTable @register="registerTable">
      <template #toolbar>
        <a-button type="primary" @click="openModal">
          添加
        </a-button>
      </template>
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'action'">
          <TableAction
            :actions="[
              {
                label: '删除',
                icon: 'ic:outline-delete-outline',
                onClick: handleDelete.bind(null, record),
              },
            ]"
          />
        </template>
      </template>
    </BasicTable>
    <BasicModal @register="registerAdd" v-bind="$attrs" title="添加" :helpMessage="['添加记录']"
                width="700px" @ok="onCloseModal">
      <BasicForm @register="registerForm" @submit="handleSubmit"/>
    </BasicModal>
  </div>

</template>

<script setup lang="ts">
import {BasicColumn, BasicTable, FormSchema, useTable,TableAction} from "/@/components/Table";
import {userInfoDelete, userInfoInsert, userInfoList} from "/@/pages/api/userInfo";
import {formatToDateTime} from "@/utils/dateUtil";
import {useMessage} from "@/hooks/web/useMessage";
import { BasicModal, useModal } from "@/components/Modal";
import {BasicForm,useForm} from "@/components/Form";

const { createMessage, createConfirm } = useMessage();
const {success} = createMessage;
const [registerAdd, {openModal, closeModal}] = useModal();

const schemas: FormSchema[] = [
  {
    field: `name`,
    component: 'Input',
    label: `名称`,
    required: true,
  },
  {
    field: `nickname`,
    component: 'Input',
    label: `昵称`,
    required: true,
  },

];

const handleSubmit = (values: Recordable) => {
  userInfoInsert(values).then(res => {
    // 插入成功
    success("Success message");
    reload();
    resetFields();
  });
};

const onCloseModal = () => {
  closeModal();
  submit();
};


const [registerForm, {submit, resetFields}] = useForm({
  schemas,
  showActionButtonGroup: false,
  labelWidth: 120,
  size: "large",
  baseColProps: {
    xs: 22
  }
});


const columns: BasicColumn[] = [
    {
      title: "主键",
      dataIndex: "id",
    },
    {
      title: "名称",
      dataIndex: "name",
    },
    {
      title: "昵称",
      dataIndex: "nickname",
    },
    {
      title: "创建时间",
      dataIndex: "createTime",
    },

];


const [registerTable, {reload}] = useTable({
  canResize: true,
  loading: false,
  striped: false,
  bordered: false,
  showTableSetting: false,
  useSearchForm: true,
  title: '用户列表',
  titleHelpMessage: "温馨提醒",
  formConfig: {
    autoSubmitOnEnter: true,
    // 关闭折叠
    labelWidth: 50,
    schemas: [
      {
        field: `name`,
        label: `名称`,
        component: 'Input',
      },
      {
        field: `nickname`,
        label: `昵称`,
        component: 'Input',
      },

    ],
  },

  api: async (params) => {
    let data = await userInfoList(params)
    return {items: data.list, total: data.total}
  },
  columns: columns,
  pagination: {pageSize: 10},
  actionColumn: {
    width: 160,
    title: "Action",
    dataIndex: "action"
    // slots: { customRender: 'action' },
  }
});


function handleDelete(record: Recordable) {
  createConfirm({
    iconType: "error",
    title: "提示",
    content: "你正在进行删除操作...",
    onOk: async () => {
      userInfoDelete([record.id]).then(res => {
        success("删除成功");
        reload();
      });
    }
  });
}

</script>

<style scoped lang="less">

</style>
