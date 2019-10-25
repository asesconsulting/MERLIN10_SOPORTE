<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_Rest_CO</name>
   <tag></tag>
   <elementGuidId>2da52706-6213-46ce-bec2-0955d8146c88</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003creqDireccion\u003e\n \u003cxdireccion\u003e\n  \u003ccalle\u003eSan Martin 500\u003c/calle\u003e\n  \u003cnumero\u003e\u003c/numero\u003e\n  \u003cpiso\u003e\u003c/piso\u003e\n  \u003cdepto\u003e\u003c/depto\u003e\n  \u003cbarrio\u003e\u003c/barrio\u003e\n  \u003clocalidad\u003ecaba\u003c/localidad\u003e\n  \u003cpartido\u003e\u003c/partido\u003e\n  \u003cprovincia\u003e\u003c/provincia\u003e\n  \u003ccp\u003e\u003c/cp\u003e\n\u003cclientAccessCode\u003e4192c741e308f999d8aa2a226198102a\u003c/clientAccessCode\u003e\n \u003c/xdireccion\u003e\n\u003c/reqDireccion\u003e&quot;,
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
assertThat(response.getResponseText()).contains('&lt;calle>SAN MARTIN&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;numero>500&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;piso>&lt;/piso>')
assertThat(response.getResponseText()).contains('&lt;depto>&lt;/depto>')
assertThat(response.getResponseText()).contains('&lt;localidad>CIUDAD AUTONOMA BUENOS AIRES&lt;/localidad>')
assertThat(response.getResponseText()).contains('&lt;barrio>SAN NICOLAS&lt;/barrio>')
assertThat(response.getResponseText()).contains('&lt;partido>CAPITAL FEDERAL&lt;/partido>')
assertThat(response.getResponseText()).contains('&lt;provincia>CAPITAL FEDERAL&lt;/provincia>')
assertThat(response.getResponseText()).contains('&lt;cp>1004&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;idmax>10314173&lt;/idmax>')
assertThat(response.getResponseText()).contains('&lt;x>-58.373966&lt;/x>')
assertThat(response.getResponseText()).contains('&lt;y>-34.602308&lt;/y>')
assertThat(response.getResponseText()).contains('&lt;desde>1&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>1400&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;tipoGeo>1&lt;/tipoGeo>')
assertThat(response.getResponseText()).contains('&lt;NISE>8&lt;/NISE>')
assertThat(response.getResponseText()).contains('&lt;longitud>-58.373966&lt;/longitud>')
assertThat(response.getResponseText()).contains('&lt;nivelRiesgo>&lt;/nivelRiesgo>')
assertThat(response.getResponseText()).contains('&lt;zonaRiesgo>N&lt;/zonaRiesgo>')
assertThat(response.getResponseText()).contains('&lt;longitudLocalidad>-58.445288&lt;/longitudLocalidad>')
assertThat(response.getResponseText()).contains('&lt;entreCalle2>GRAL JUAN LAVALLE&lt;/entreCalle2>')
assertThat(response.getResponseText()).contains('&lt;entreCalle1>AVDA CORRIENTES&lt;/entreCalle1>')
assertThat(response.getResponseText()).contains('&lt;concepto>&lt;/concepto>')
assertThat(response.getResponseText()).contains('&lt;latitudLocalidad>-34.607161&lt;/latitudLocalidad>')
assertThat(response.getResponseText()).contains('&lt;puerta>SI&lt;/puerta>')
assertThat(response.getResponseText()).contains('&lt;inMaq>0&lt;/inMaq>')
assertThat(response.getResponseText()).contains('&lt;cpa>C1004AAJ&lt;/cpa>')
assertThat(response.getResponseText()).contains('&lt;tipoVivienda>otro&lt;/tipoVivienda>')
assertThat(response.getResponseText()).contains('&lt;latitud>-34.602308&lt;/latitud>')
assertThat(response.getResponseText()).contains('&lt;empresa>NO&lt;/empresa>')
assertThat(response.getResponseText()).contains('&lt;zonaFranca>N&lt;/zonaFranca>')



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
