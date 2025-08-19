package com.daniel.deliveryrouting.data.api

import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import android.util.Log
import java.util.concurrent.TimeUnit

class ColisSoapClient {
    
    private companion object {
        const val NAMESPACE = "http://www.navettews.org/"
        const val URL = "http://sd3.danem.fr/cpdistri/plutonservice.asmx"
        const val SOAP_ACTION = "http://www.navettews.org/BonjourDistri"
        const val METHOD_NAME = "BonjourDistri"
    }
    
    suspend fun bonjourDistri(login: String, imei: String): Result<String> = withContext(Dispatchers.IO) {
        try {
            Log.d("ColisSoapClient", "Calling SOAP BonjourDistri with login: $login, imei: $imei")
            
            // Crear cliente HTTP para SOAP
            val client = OkHttpClient.Builder()
                .connectTimeout(30, TimeUnit.SECONDS)
                .readTimeout(30, TimeUnit.SECONDS)
                .writeTimeout(30, TimeUnit.SECONDS)
                .build()
            
            // Construir SOAP request manualmente
            val soapBody = """
                <?xml version="1.0" encoding="utf-8"?>
                <soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" 
                               xmlns:xsd="http://www.w3.org/2001/XMLSchema" 
                               xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
                    <soap:Body>
                        <BonjourDistri xmlns="http://www.navettews.org/">
                            <login>${login};SM-G988B</login>
                            <imei>${imei}</imei>
                        </BonjourDistri>
                    </soap:Body>
                </soap:Envelope>
            """.trimIndent()
            
            val request = Request.Builder()
                .url(URL)
                .addHeader("SOAPAction", SOAP_ACTION)
                .addHeader("Content-Type", "text/xml; charset=utf-8")
                .post(RequestBody.create("text/xml; charset=utf-8".toMediaType(), soapBody))
                .build()
            
            Log.d("ColisSoapClient", "SOAP Request: $soapBody")
            Log.d("ColisSoapClient", "SOAP URL: $URL")
            Log.d("ColisSoapClient", "SOAP Action: $SOAP_ACTION")
            
            val response = client.newCall(request).execute()
            val responseBody = response.body?.string() ?: "No response"
            
            Log.d("ColisSoapClient", "SOAP Response Code: ${response.code}")
            Log.d("ColisSoapClient", "SOAP Response: $responseBody")
            
            if (response.isSuccessful) {
                Result.success(responseBody)
            } else {
                Result.failure(Exception("HTTP ${response.code}: ${response.message}"))
            }
            
        } catch (e: Exception) {
            Log.e("ColisSoapClient", "SOAP BonjourDistri failed: ${e.message}", e)
            Result.failure(e)
        }
    }
}
