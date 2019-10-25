<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_Rest_NE_Locker</name>
   <tag></tag>
   <elementGuidId>7f4af856-a138-4a6e-a110-91ce01bd3144</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003creqDireccion\u003e\n \u003cxdireccion\u003e\n  \u003ccalle\u003epickup gaudard 2629\u003c/calle\u003e\n  \u003cnumero\u003e\u003c/numero\u003e\n  \u003cpiso\u003e\u003c/piso\u003e\n  \u003cdepto\u003e\u003c/depto\u003e\n  \u003cbarrio\u003e\u003c/barrio\u003e\n  \u003clocalidad\u003erio Cuarto\u003c/localidad\u003e\n  \u003cpartido\u003e\u003c/partido\u003e\n  \u003cprovincia\u003e\u003c/provincia\u003e\n  \u003ccp\u003e\u003c/cp\u003e\n\u003cclientAccessCode\u003ef35c5a599b268a499da6e4cae7f7a265\u003c/clientAccessCode\u003e\n \u003c/xdireccion\u003e\n\u003c/reqDireccion\u003e&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${RestService_1_5}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.RestService_1_5</defaultValue>
      <description></description>
      <id>4e2878f5-aea7-4e55-9365-10b5a094d43a</id>
      <masked>false</masked>
      <name>RestService_1_5</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


assertThat(response.getResponseText()).contains('&lt;estado>CO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>SM&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;cantdudas>0&lt;/cantdudas>')
assertThat(response.getResponseText()).contains('&lt;observaciones>&lt;/observaciones>')
assertThat(response.getResponseText()).contains('&lt;calle>PICKUP(ANDREANI-A-RIO CUARTO) GRAL PAZ&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;numero>1175&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;piso>&lt;/piso>')
assertThat(response.getResponseText()).contains('&lt;depto>&lt;/depto>')
assertThat(response.getResponseText()).contains('&lt;localidad>RIO CUARTO&lt;/localidad>')
assertThat(response.getResponseText()).contains('&lt;barrio>CENTRO&lt;/barrio>')
assertThat(response.getResponseText()).contains('&lt;partido>CORDOBA&lt;/partido>')
assertThat(response.getResponseText()).contains('&lt;provincia>CORDOBA&lt;/provincia>')
assertThat(response.getResponseText()).contains('&lt;cp>5800&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;idmax>&lt;/idmax>')
assertThat(response.getResponseText()).contains('&lt;x>-64.348816&lt;/x>')
assertThat(response.getResponseText()).contains('&lt;y>-33.128700&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;desde>0&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>0&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;latitudLocalidad>-33.126686&lt;/latitudLocalidad>')
assertThat(response.getResponseText()).contains('&lt;latitud>-33.1287&lt;/latitud>')
assertThat(response.getResponseText()).contains('&lt;longitudLocalidad>-64.34879&lt;/longitudLocalidad>')
assertThat(response.getResponseText()).contains('&lt;puerta>NO RELEVADO&lt;/puerta>')
assertThat(response.getResponseText()).contains('&lt;longitud>-64.348816&lt;/longitud>')
assertThat(response.getResponseText()).contains('&lt;cpa>X5800ADA&lt;/cpa>')
assertThat(response.getResponseText()).contains('&lt;zonaRiesgo>N&lt;/zonaRiesgo>')
assertThat(response.getResponseText()).contains('&lt;entreCalle2>PEDERNERA&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;tipoGeo>1&lt;/tipoGeo>')
assertThat(response.getResponseText()).contains('&lt;nise>6&lt;/nise>')
assertThat(response.getResponseText()).contains('&lt;horarios>&lt;/horarios>')



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
