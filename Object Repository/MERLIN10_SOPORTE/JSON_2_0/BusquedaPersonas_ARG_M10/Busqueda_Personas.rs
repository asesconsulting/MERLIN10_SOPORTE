<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Busqueda_Personas</name>
   <tag></tag>
   <elementGuidId>d858aa8c-3f77-4051-9d34-8897c5bb53ea</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded; charset=utf-8</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${PersonSearchJson}?clientAccessCode=a1edeae2a5bd4cde241fdfdb193ca13c&amp;tributaryNumber=20238659079</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PersonSearchJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>PersonSearchJson</name>
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


WS.verifyElementPropertyValue(response, 'status', &quot;EN&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nPerson.numberAlternativePerson', &quot;1&quot;)
assertThat(response.getResponseText()).contains('&quot;documentType&quot;:&quot;96&quot;,&quot;documentNumber&quot;:&quot;23865907&quot;,&quot;tributaryType&quot;:&quot;80&quot;,&quot;tributaryNumber&quot;:&quot;20238659079&quot;,&quot;name&quot;:&quot;FEDERICO ANDRES&quot;,&quot;lastName&quot;:&quot;MOGLIA&quot;,&quot;gender&quot;:&quot;M&quot;,&quot;birthDate&quot;:&quot;29/03/1974&quot;,&quot;street&quot;:&quot;AVDA INDEPENDENCIA&quot;,&quot;houseNumber&quot;:&quot;650&quot;,&quot;floor&quot;:&quot;9&quot;,&quot;unit&quot;:&quot;D&quot;,&quot;level2&quot;:&quot;CAPITAL FEDERAL&quot;,&quot;level4&quot;:&quot;CIUDAD AUTONOMA BUENOS AIRES&quot;,&quot;postalCode&quot;:&quot;1099&quot;')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
