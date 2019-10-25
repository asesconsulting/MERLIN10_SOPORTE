<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiatorios_VA_Empresa</name>
   <tag></tag>
   <elementGuidId>247a2608-6fe9-44e8-ab70-7b9ec9cf4bb7</elementGuidId>
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
   <restUrl>${FiliatoriosJson}?tributaryNumber=30676235309&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;name=</restUrl>
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



assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;literalDistance&quot;,&quot;value&quot;:&quot;100.00&quot;}')
WS.verifyElementPropertyValue(response, 'status', &quot;OK&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentTypeFlg', &quot;NA&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.documentNumberFlg', &quot;NA&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryType', &quot;80&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryTypeFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumber', &quot;30676235309&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.tributaryNumberFlg', &quot;VA&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorType', &quot;C&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.contributorTypeFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastName', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.lastNameFlg', &quot;NI&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.name', &quot;ASES CONSULTING S.A.&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.nameFlg', &quot;AP&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.gender', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.genderFlg', &quot;NA&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDate', &quot;01/10/1993&quot;)
WS.verifyElementPropertyValue(response, 'nFiliation.birthDateFlg', &quot;AP&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
