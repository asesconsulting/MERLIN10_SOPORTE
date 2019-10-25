<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ConsultaMaestros_CO</name>
   <tag></tag>
   <elementGuidId>d0b50f3a-17b8-4efd-af4d-4c3a3a17b4e6</elementGuidId>
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
   <restUrl>${ConsultaMaestrosJson}?clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;level1=AR&amp;level2=Buenos%20Aires&amp;level4=florencio%20varela&amp;stringSearch=martin</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.ConsultaMaestrosJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>ConsultaMaestrosJson</name>
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
WS.verifyElementPropertyValue(response, 'totalRecords', 5)

assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level4&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;AVENIDA GENERAL JOSE DE SAN MARTIN&quot;,&quot;postalCode&quot;:&quot;1888&quot;,&quot;streetNumberFrom&quot;:&quot;1&quot;,&quot;streetNumberTo&quot;:&quot;3400&quot;,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;shortStreet&quot;,&quot;value&quot;:&quot;&quot;}],&quot;segmentList&quot;:[],&quot;synonymous&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level4&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;DIAGONAL GENERAL MARTIN MIGUEL DE GUEMES&quot;,&quot;postalCode&quot;:&quot;1888&quot;,&quot;streetNumberFrom&quot;:&quot;1901&quot;,&quot;streetNumberTo&quot;:&quot;4300&quot;,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;shortStreet&quot;,&quot;value&quot;:&quot;&quot;}],&quot;segmentList&quot;:[],&quot;synonymous&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level4&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;GENERAL JUAN MARTIN DE PUEYRREDON&quot;,&quot;postalCode&quot;:&quot;1888&quot;,&quot;streetNumberFrom&quot;:&quot;1&quot;,&quot;streetNumberTo&quot;:&quot;400&quot;,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;shortStreet&quot;,&quot;value&quot;:&quot;&quot;}],&quot;segmentList&quot;:[],&quot;synonymous&quot;:&quot;CALLE 358&quot;}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level4&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;GRANADEROS DE SAN MARTIN&quot;,&quot;postalCode&quot;:&quot;1888&quot;,&quot;streetNumberFrom&quot;:&quot;1&quot;,&quot;streetNumberTo&quot;:&quot;2000&quot;,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;shortStreet&quot;,&quot;value&quot;:&quot;&quot;}],&quot;segmentList&quot;:[],&quot;synonymous&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;level1&quot;:&quot;AR&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level4&quot;:&quot;FLORENCIO VARELA&quot;,&quot;level5&quot;:&quot;&quot;,&quot;suggestedItem&quot;:&quot;ISLA MARTIN GARCIA&quot;,&quot;postalCode&quot;:&quot;1888&quot;,&quot;streetNumberFrom&quot;:&quot;901&quot;,&quot;streetNumberTo&quot;:&quot;1700&quot;,&quot;merlinCustomValues&quot;:[{&quot;name&quot;:&quot;shortStreet&quot;,&quot;value&quot;:&quot;&quot;}],&quot;segmentList&quot;:[],&quot;synonymous&quot;:&quot;&quot;}')


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
