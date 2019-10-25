<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiatorios_DU</name>
   <tag></tag>
   <elementGuidId>5a89970f-afa8-4078-aa46-81bf1a598c77</elementGuidId>
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
   <restUrl>${FiliatoriosJson}?documentNumber=4356144&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;name=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.FiliatoriosJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>FiliatoriosJson</name>
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


WS.verifyElementPropertyValue(response, 'status', &quot;DU&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentTypeFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumber', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumberFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryTypeFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumber', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumberFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorTypeFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastName', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastNameFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.name', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.nameFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.gender', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.genderFlg', &quot;NV&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDate', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDateFlg', &quot;NV&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
