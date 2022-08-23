/**
 * notifier
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.1
 * Contact: 
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package notifier.models


import com.squareup.moshi.Json

/**
 * 
 *
 * @param message 
 * @param userId 
 * @param subject 
 */

data class SendNotificationBody (

    @Json(name = "message")
    val message: kotlin.String,

    @Json(name = "user_id")
    val userId: kotlin.String,

    @Json(name = "subject")
    val subject: kotlin.String

)
