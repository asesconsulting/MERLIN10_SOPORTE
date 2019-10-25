<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Telefono_SD</name>
   <tag></tag>
   <elementGuidId>02cc3e74-502f-4076-a82e-fa003c6c0bf5</elementGuidId>
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
   <restUrl>${PhoneJson}?phoneNumber=123&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PhoneJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>PhoneJson</name>
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



WS.verifyElementPropertyValue(response, 'status', &quot;SD&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.level2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.level3', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.level4', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.postalCode', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.ddi', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.ddn', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.characteristic', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.phoneNumber', &quot;123&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.validated', &quot;NO&quot;)
WS.verifyElementPropertyValue(response, 'nPhone.additionalData', &quot;&quot;)

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;fullCellphoneNumber&quot;,&quot;value&quot;:&quot;123&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;telefonoCompleto&quot;,&quot;value&quot;:&quot;123&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;directory&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;doNotCallRegistry&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;PORTAL&quot;,&quot;value&quot;:&quot;PORTAL&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;fullPhone&quot;,&quot;value&quot;:&quot;123&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
