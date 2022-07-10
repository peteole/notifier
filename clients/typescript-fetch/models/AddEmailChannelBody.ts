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
 * @interface AddEmailChannelBody
 */
export interface AddEmailChannelBody {
    /**
     * 
     * @type {string}
     * @memberof AddEmailChannelBody
     */
    userId: string;
    /**
     * 
     * @type {string}
     * @memberof AddEmailChannelBody
     */
    email: string;
}

/**
 * Check if a given object implements the AddEmailChannelBody interface.
 */
export function instanceOfAddEmailChannelBody(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "userId" in value;
    isInstance = isInstance && "email" in value;

    return isInstance;
}

export function AddEmailChannelBodyFromJSON(json: any): AddEmailChannelBody {
    return AddEmailChannelBodyFromJSONTyped(json, false);
}

export function AddEmailChannelBodyFromJSONTyped(json: any, ignoreDiscriminator: boolean): AddEmailChannelBody {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'userId': json['user_id'],
        'email': json['email'],
    };
}

export function AddEmailChannelBodyToJSON(value?: AddEmailChannelBody | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'user_id': value.userId,
        'email': value.email,
    };
}

