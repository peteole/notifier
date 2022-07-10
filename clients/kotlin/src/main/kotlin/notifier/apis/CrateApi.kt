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

package notifier.apis

import java.io.IOException
import okhttp3.OkHttpClient

import notifier.models.AddEmailChannelBody
import notifier.models.AddTelegramChannelBody
import notifier.models.SendNotificationBody

import com.squareup.moshi.Json

import notifier.infrastructure.ApiClient
import notifier.infrastructure.ApiResponse
import notifier.infrastructure.ClientException
import notifier.infrastructure.ClientError
import notifier.infrastructure.ServerException
import notifier.infrastructure.ServerError
import notifier.infrastructure.MultiValueMap
import notifier.infrastructure.PartConfig
import notifier.infrastructure.RequestConfig
import notifier.infrastructure.RequestMethod
import notifier.infrastructure.ResponseType
import notifier.infrastructure.Success
import notifier.infrastructure.toMultiValue

class CrateApi(basePath: kotlin.String = defaultBasePath, client: OkHttpClient = ApiClient.defaultClient) : ApiClient(basePath, client) {
    companion object {
        @JvmStatic
        val defaultBasePath: String by lazy {
            System.getProperties().getProperty(ApiClient.baseUrlKey, "http://localhost")
        }
    }

    /**
     * Add email channel
     * Add email channel  Add email notification channel for user 
     * @param addEmailChannelBody 
     * @return void
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun handleAddEmailChannel(addEmailChannelBody: AddEmailChannelBody) : Unit {
        val localVarResponse = handleAddEmailChannelWithHttpInfo(addEmailChannelBody = addEmailChannelBody)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> Unit
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Add email channel
     * Add email channel  Add email notification channel for user 
     * @param addEmailChannelBody 
     * @return ApiResponse<Unit?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Throws(IllegalStateException::class, IOException::class)
    fun handleAddEmailChannelWithHttpInfo(addEmailChannelBody: AddEmailChannelBody) : ApiResponse<Unit?> {
        val localVariableConfig = handleAddEmailChannelRequestConfig(addEmailChannelBody = addEmailChannelBody)

        return request<AddEmailChannelBody, Unit>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation handleAddEmailChannel
     *
     * @param addEmailChannelBody 
     * @return RequestConfig
     */
    fun handleAddEmailChannelRequestConfig(addEmailChannelBody: AddEmailChannelBody) : RequestConfig<AddEmailChannelBody> {
        val localVariableBody = addEmailChannelBody
        val localVariableQuery: MultiValueMap = mutableMapOf()
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Content-Type"] = "application/json"
        
        return RequestConfig(
            method = RequestMethod.POST,
            path = "/add_channel/email",
            query = localVariableQuery,
            headers = localVariableHeaders,
            body = localVariableBody
        )
    }

    /**
     * Add telegram channel
     * Add telegram channel  Add telegram notification channel for user 
     * @param addTelegramChannelBody 
     * @return void
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun handleAddTelegramChannel(addTelegramChannelBody: AddTelegramChannelBody) : Unit {
        val localVarResponse = handleAddTelegramChannelWithHttpInfo(addTelegramChannelBody = addTelegramChannelBody)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> Unit
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Add telegram channel
     * Add telegram channel  Add telegram notification channel for user 
     * @param addTelegramChannelBody 
     * @return ApiResponse<Unit?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Throws(IllegalStateException::class, IOException::class)
    fun handleAddTelegramChannelWithHttpInfo(addTelegramChannelBody: AddTelegramChannelBody) : ApiResponse<Unit?> {
        val localVariableConfig = handleAddTelegramChannelRequestConfig(addTelegramChannelBody = addTelegramChannelBody)

        return request<AddTelegramChannelBody, Unit>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation handleAddTelegramChannel
     *
     * @param addTelegramChannelBody 
     * @return RequestConfig
     */
    fun handleAddTelegramChannelRequestConfig(addTelegramChannelBody: AddTelegramChannelBody) : RequestConfig<AddTelegramChannelBody> {
        val localVariableBody = addTelegramChannelBody
        val localVariableQuery: MultiValueMap = mutableMapOf()
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Content-Type"] = "application/json"
        
        return RequestConfig(
            method = RequestMethod.POST,
            path = "/add_channel/telegram",
            query = localVariableQuery,
            headers = localVariableHeaders,
            body = localVariableBody
        )
    }

    /**
     * Send notification
     * Send notification  send notification to user with given id on all channels registered for that user 
     * @param sendNotificationBody 
     * @return void
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun handleSendNotification(sendNotificationBody: SendNotificationBody) : Unit {
        val localVarResponse = handleSendNotificationWithHttpInfo(sendNotificationBody = sendNotificationBody)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> Unit
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Send notification
     * Send notification  send notification to user with given id on all channels registered for that user 
     * @param sendNotificationBody 
     * @return ApiResponse<Unit?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Throws(IllegalStateException::class, IOException::class)
    fun handleSendNotificationWithHttpInfo(sendNotificationBody: SendNotificationBody) : ApiResponse<Unit?> {
        val localVariableConfig = handleSendNotificationRequestConfig(sendNotificationBody = sendNotificationBody)

        return request<SendNotificationBody, Unit>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation handleSendNotification
     *
     * @param sendNotificationBody 
     * @return RequestConfig
     */
    fun handleSendNotificationRequestConfig(sendNotificationBody: SendNotificationBody) : RequestConfig<SendNotificationBody> {
        val localVariableBody = sendNotificationBody
        val localVariableQuery: MultiValueMap = mutableMapOf()
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Content-Type"] = "application/json"
        
        return RequestConfig(
            method = RequestMethod.POST,
            path = "/notify",
            query = localVariableQuery,
            headers = localVariableHeaders,
            body = localVariableBody
        )
    }

}
