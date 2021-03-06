<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_Rest_CO_Macro1</name>
   <tag></tag>
   <elementGuidId>ac318f49-a039-42d1-baad-725f2c6332f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003creqDireccion\u003e\n \u003cxdireccion\u003e\n  \u003ccalle\u003earias 2253\u003c/calle\u003e\n  \u003cnumero\u003e\u003c/numero\u003e\n  \u003cpiso\u003e\u003c/piso\u003e\n  \u003cdepto\u003e\u003c/depto\u003e\n  \u003cbarrio\u003e\u003c/barrio\u003e\n  \u003clocalidad\u003ecaba\u003c/localidad\u003e\n  \u003cpartido\u003e\u003c/partido\u003e\n  \u003cprovincia\u003e\u003c/provincia\u003e\n  \u003ccp\u003e\u003c/cp\u003e\n\u003cclientAccessCode\u003e7d6bb345ce19a5c55627b8cb1f7f506b\u003c/clientAccessCode\u003e\n \u003c/xdireccion\u003e\n\u003c/reqDireccion\u003e&quot;,
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
assertThat(response.getResponseText()).contains('&lt;motivo>&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;cantdudas>0&lt;/cantdudas>')
assertThat(response.getResponseText()).contains('&lt;observaciones>&lt;/observaciones>')
assertThat(response.getResponseText()).contains('&lt;calle>CNEL M E ARIAS&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;numero>2253&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;piso>&lt;/piso>')
assertThat(response.getResponseText()).contains('&lt;depto>&lt;/depto>')
assertThat(response.getResponseText()).contains('&lt;localidad>CAPITAL FEDERAL&lt;/localidad>')
assertThat(response.getResponseText()).contains('&lt;barrio>&lt;/barrio>')
assertThat(response.getResponseText()).contains('&lt;partido>CAPITAL FEDERAL&lt;/partido>')
assertThat(response.getResponseText()).contains('&lt;provincia>CAPITAL FEDERAL&lt;/provincia>')
assertThat(response.getResponseText()).contains('&lt;cp>1429&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;idmax>10285475&lt;/idmax>')
assertThat(response.getResponseText()).contains('&lt;x>-58.471568&lt;/x>')
assertThat(response.getResponseText()).contains('&lt;y>-34.541360&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;desde>1601&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>5000&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;tipoGeo>1&lt;/tipoGeo>')
assertThat(response.getResponseText()).contains('&lt;NISE>9&lt;/NISE>')
assertThat(response.getResponseText()).contains('&lt;tipoVivienda>casa&lt;/tipoVivienda>')
assertThat(response.getResponseText()).contains('&lt;cpa>C1429DXK&lt;/cpa>')
assertThat(response.getResponseText()).contains('&lt;AudLoc>S&lt;/AudLoc>')
assertThat(response.getResponseText()).contains('&lt;AudNum>N&lt;/AudNum>')
assertThat(response.getResponseText()).contains('&lt;zonaRiesgo>N&lt;/zonaRiesgo>')
assertThat(response.getResponseText()).contains('&lt;AudObs>N&lt;/AudObs>')
assertThat(response.getResponseText()).contains('&lt;entreCalle1>CUBA&lt;/entreCalle1>')
assertThat(response.getResponseText()).contains('&lt;TBZ>NA&lt;/TBZ>')
assertThat(response.getResponseText()).contains('&lt;AudCP>S&lt;/AudCP>')
assertThat(response.getResponseText()).contains('&lt;entreCalle2>VUELTA DE OBLIGADO&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;REFERENCIA>&lt;/REFERENCIA>')
assertThat(response.getResponseText()).contains('&lt;usuario>MERLIN&lt;/usuario>')
assertThat(response.getResponseText()).contains('&lt;puerta>SI&lt;/puerta>')
assertThat(response.getResponseText()).contains('&lt;longitud>-58.471568&lt;/longitud>')
assertThat(response.getResponseText()).contains('&lt;latitud>-34.541360&lt;/latitud>')
assertThat(response.getResponseText()).contains('&lt;empresa>NO&lt;/empresa>')


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
