/** Types generated for queries found in "src/notifications/notifications.sql" */
import { PreparedQuery } from '@pgtyped/query';

export type notification_type = 'deadline' | 'notification' | 'reminder';

export type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

/** 'SendNotifications' parameters type */
export interface ISendNotificationsParams {
  notifications: readonly ({
    user_id: number,
    payload: Json,
    type: notification_type
  })[];
}

/** 'SendNotifications' return type */
export interface ISendNotificationsResult {
  notification_id: number;
}

/** 'SendNotifications' query type */
export interface ISendNotificationsQuery {
  params: ISendNotificationsParams;
  result: ISendNotificationsResult;
}

const sendNotificationsIR: any = {"name":"SendNotifications","params":[{"name":"notifications","codeRefs":{"defined":{"a":40,"b":52,"line":3,"col":9},"used":[{"a":155,"b":167,"line":6,"col":8}]},"transform":{"type":"pick_array_spread","keys":[{"name":"user_id","required":true},{"name":"payload","required":true},{"name":"type","required":true}]},"required":false}],"usedParamSet":{"notifications":true},"statement":{"body":"INSERT INTO notifications (user_id, payload, type)\r\nVALUES :notifications RETURNING id as notification_id","loc":{"a":95,"b":199,"line":5,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * INSERT INTO notifications (user_id, payload, type)
 * VALUES :notifications RETURNING id as notification_id
 * ```
 */
export const sendNotifications = new PreparedQuery<ISendNotificationsParams,ISendNotificationsResult>(sendNotificationsIR);


/** 'GetNotifications' parameters type */
export interface IGetNotificationsParams {
  userId: number | null | void;
}

/** 'GetNotifications' return type */
export interface IGetNotificationsResult {
  id: number;
  user_id: number | null;
  payload: Json;
  type: notification_type;
}

/** 'GetNotifications' query type */
export interface IGetNotificationsQuery {
  params: IGetNotificationsParams;
  result: IGetNotificationsResult;
}

const getNotificationsIR: any = {"name":"GetNotifications","params":[{"name":"userId","required":false,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":285,"b":290,"line":11,"col":18}]}}],"usedParamSet":{"userId":true},"statement":{"body":"SELECT *\r\n  FROM notifications\r\n WHERE user_id = :userId","loc":{"a":235,"b":290,"line":9,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * SELECT *
 *   FROM notifications
 *  WHERE user_id = :userId
 * ```
 */
export const getNotifications = new PreparedQuery<IGetNotificationsParams,IGetNotificationsResult>(getNotificationsIR);


/** 'ThresholdFrogs' parameters type */
export interface IThresholdFrogsParams {
  numFrogs: number;
}

/** 'ThresholdFrogs' return type */
export interface IThresholdFrogsResult {
  user_name: string;
  payload: Json;
  type: notification_type;
}

/** 'ThresholdFrogs' query type */
export interface IThresholdFrogsQuery {
  params: IThresholdFrogsParams;
  result: IThresholdFrogsResult;
}

const thresholdFrogsIR: any = {"name":"ThresholdFrogs","params":[{"name":"numFrogs","required":true,"transform":{"type":"scalar"},"codeRefs":{"used":[{"a":477,"b":485,"line":20,"col":46}]}}],"usedParamSet":{"numFrogs":true},"statement":{"body":"SELECT u.user_name, n.payload, n.type\r\nFROM notifications n\r\nINNER JOIN users u on n.user_id = u.id\r\nWHERE CAST (n.payload->'num_frogs' AS int) > :numFrogs!","loc":{"a":330,"b":485,"line":17,"col":0}}};

/**
 * Query generated from SQL:
 * ```
 * SELECT u.user_name, n.payload, n.type
 * FROM notifications n
 * INNER JOIN users u on n.user_id = u.id
 * WHERE CAST (n.payload->'num_frogs' AS int) > :numFrogs!
 * ```
 */
export const thresholdFrogs = new PreparedQuery<IThresholdFrogsParams,IThresholdFrogsResult>(thresholdFrogsIR);


