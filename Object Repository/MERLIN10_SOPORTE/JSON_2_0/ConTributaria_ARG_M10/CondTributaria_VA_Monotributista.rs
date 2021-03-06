<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CondTributaria_VA_Monotributista</name>
   <tag></tag>
   <elementGuidId>9b2e4904-47a8-4112-8639-d687466f18a5</elementGuidId>
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
   <restUrl>${TributariaJson}?clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;cuit=27165525413</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.TributariaJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>TributariaJson</name>
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


WS.verifyElementPropertyValue(response, 'status', &quot;VA&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.cuit', &quot;27165525413&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.denomination', &quot;SOSA MARTA INES               &quot;)
WS.verifyElementPropertyValue(response, 'nCuit.denominationFlg', &quot;NN&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.aernings', &quot;AC&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.iva', &quot;AC&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.uniqueTribute', &quot;NI&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.uniqueTributeActivity', &quot;00&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.employer', &quot;N&quot;)
WS.verifyElementPropertyValue(response, 'nCuit.memberOfSociety', &quot;N&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
