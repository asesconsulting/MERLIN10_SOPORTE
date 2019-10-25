<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_VA_Telefonica</name>
   <tag></tag>
   <elementGuidId>e776199b-b550-44db-937b-a5141574bb57</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003c?xml version\u003d\&quot;1.0\&quot; encoding\u003d\&quot;UTF-8\&quot;?\u003e\n\u003creqMaestrosCalle\u003e\n \u003cnivel1\u003eAR\u003c/nivel1\u003e\n \u003cnivel2\u003eBUENOS AIRES\u003c/nivel2\u003e\n \u003cnivel3\u003e\u003c/nivel3\u003e\n \u003cnivel4\u003eVICENTE LOPEZ\u003c/nivel4\u003e\n \u003ccalle\u003eSAN\u003c/calle\u003e\n \u003cclientAccessCode\u003eaf36223848761cba417718921cd2f9df\u003c/clientAccessCode\u003e\n\u003c/reqMaestrosCalle\u003e&quot;,
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


assertThat(response.getResponseText()).contains('&lt;estado>LISTADO_CORRECTO&lt;/estado>')
assertThat(response.getResponseText()).contains('&lt;motivo>SM&lt;/motivo>')
assertThat(response.getResponseText()).contains('&lt;totalRecords>4&lt;/totalRecords>')
assertThat(response.getResponseText()).contains('&lt;calle>AVENIDA GENERAL JOSE DE SAN MARTIN&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;nivel2>BUENOS AIRES&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel4>VICENTE LOPEZ&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;desde>301&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>1500&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;cp>1638&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;calle>DOCTOR LISANDRO DE LA TORRE&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;nivel2>BUENOS AIRES&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel4>VICENTE LOPEZ&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;desde>151&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>2000&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;cp>1638&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;calle>FLORENCIO SANCHEZ&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;nivel2>BUENOS AIRES&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel4>VICENTE LOPEZ&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;desde>1901&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>2000&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;cp>1638&lt;/cp>')
assertThat(response.getResponseText()).contains('&lt;calle>SANTA ROSA&lt;/calle>')
assertThat(response.getResponseText()).contains('&lt;nivel2>BUENOS AIRES&lt;/nivel2>')
assertThat(response.getResponseText()).contains('&lt;nivel4>VICENTE LOPEZ&lt;/nivel4>')
assertThat(response.getResponseText()).contains('&lt;desde>1201&lt;/desde>')
assertThat(response.getResponseText()).contains('&lt;hasta>1500&lt;/hasta>')
assertThat(response.getResponseText()).contains('&lt;cp>1638&lt;/cp>')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
