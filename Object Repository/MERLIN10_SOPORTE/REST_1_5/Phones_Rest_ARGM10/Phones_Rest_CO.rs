<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Phones_Rest_CO</name>
   <tag></tag>
   <elementGuidId>ed4af918-f903-4d9c-b853-b84d7f655847</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003creqPhone\u003e\n  \u003cxtelefono\u003e\n    \u003cnivel1\u003e\u003c/nivel1\u003e\n    \u003cnumero\u003e5403471426966\u003c/numero\u003e\n    \u003cnivel4\u003eSANTA FE\u003c/nivel4\u003e\n    \u003cprefijo\u003e0347\u003c/prefijo\u003e\n    \u003cnivel2\u003eCANADA DE GOMEZ\u003c/nivel2\u003e\n    \u003ccp\u003e\u003c/cp\u003e\n\u003cclientAccessCode\u003eaea243aba41084aa098b3a70eeb63ddf\u003c/clientAccessCode\u003e\n  \u003c/xtelefono\u003e\n\u003c/reqPhone\u003e&quot;,
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
assertThat(response.getResponseText()).contains('&lt;prefijo>03471&lt;/prefijo>')
assertThat(response.getResponseText()).contains('&lt;prefcelular>&lt;/prefcelular>')
assertThat(response.getResponseText()).contains('&lt;caracteristica>42&lt;/caracteristica>')
assertThat(response.getResponseText()).contains('&lt;numero>6966&lt;/numero>')
assertThat(response.getResponseText()).contains('&lt;validado>NO&lt;/validado>')
assertThat(response.getResponseText()).contains('&lt;observaciones>NULL&lt;/observaciones>')
assertThat(response.getResponseText()).contains('&lt;TEL_COMPLETO>03471426966&lt;/TEL_COMPLETO>')
assertThat(response.getResponseText()).contains('&lt;figuraEnGuia>NO&lt;/figuraEnGuia>')
assertThat(response.getResponseText()).contains('&lt;registroNoLlame>NO&lt;/registroNoLlame>')
assertThat(response.getResponseText()).contains('&lt;CEL_COMPLETO>03471426966&lt;/CEL_COMPLETO>')
assertThat(response.getResponseText()).contains('&lt;telefonoCompleto>03471426966&lt;/telefonoCompleto>')




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
