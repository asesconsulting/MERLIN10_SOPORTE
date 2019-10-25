<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Nombre_CO</name>
   <tag></tag>
   <elementGuidId>157a1bce-c3be-454f-902f-3d09c1e3da4c</elementGuidId>
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
   <restUrl>${NombresJson}?name=Elvis%20Antonio%20Goncalves&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.NombresJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>NombresJson</name>
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



WS.verifyElementPropertyValue(response, 'status', &quot;CO&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nName.name', &quot;ELVIS ANTONIO&quot;)
WS.verifyElementPropertyValue(response, 'nName.lastName', &quot;GONCALVES&quot;)
WS.verifyElementPropertyValue(response, 'nName.personType', &quot;PF&quot;)
WS.verifyElementPropertyValue(response, 'nName.gender', &quot;M&quot;)
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;PORTAL&quot;,&quot;value&quot;:&quot;PORTAL&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
