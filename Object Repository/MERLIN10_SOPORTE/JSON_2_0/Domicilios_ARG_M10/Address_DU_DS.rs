<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DS</name>
   <tag></tag>
   <elementGuidId>f41a89af-adcf-4bee-8f9e-7462299827ba</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=BELGRANO%20629&amp;level4=SAN%20ALBERTO&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;postalCode=4105&amp;level2=TUCUMAN</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AddressJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>AddressJson</name>
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



WS.verifyElementPropertyValue(response, 'status', &quot;NE&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;AI&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;5&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;TUCUMAN&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;LULES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;EL MANANTIAL&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;AVDA DR MANUEL BELGRANO&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;629&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-26.846086&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-65.283462&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;4105&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;2&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;0&quot;)

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;puerta&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-26.849720&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-65.283060&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;1&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;cpa&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-65.283060&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-26.849720&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;1&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;N&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
