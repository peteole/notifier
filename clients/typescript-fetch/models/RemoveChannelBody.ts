/* tslint:disable */
/* eslint-disable */
/**
 * notifier
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.1
 * Contact: 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface RemoveChannelBody
 */
export interface RemoveChannelBody {
    /**
     * 
     * @type {string}
     * @memberof RemoveChannelBody
     */
    serviceId: string;
    /**
     * 
     * @type {string}
     * @memberof RemoveChannelBody
     */
    userId: string;
}

/**
 * Check if a given object implements the RemoveChannelBody interface.
 */
export function instanceOfRemoveChannelBody(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "serviceId" in value;
    isInstance = isInstance && "userId" in value;

    return isInstance;
}

export function RemoveChannelBodyFromJSON(json: any): RemoveChannelBody {
    return RemoveChannelBodyFromJSONTyped(json, false);
}

export function RemoveChannelBodyFromJSONTyped(json: any, ignoreDiscriminator: boolean): RemoveChannelBody {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'serviceId': json['service_id'],
        'userId': json['user_id'],
    };
}

export function RemoveChannelBodyToJSON(value?: RemoveChannelBody | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'service_id': value.serviceId,
        'user_id': value.userId,
    };
}

